//! # Browser Automation Service
//!
//! Provides secure browser automation using headless Chrome for web interaction tasks.
//! Implements navigation, data extraction, clicking, and form filling capabilities.

use anyhow::{Context, Result};
use headless_chrome::{Browser, LaunchOptions, Tab};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Browser automation service with shared headless Chrome instance
#[derive(Clone)]
pub struct BrowserService {
    browser: Arc<RwLock<Option<Browser>>>,
    options: BrowserOptions,
}

impl BrowserService {
    /// Create a new browser service with default options
    ///
    /// # Errors
    ///
    /// Returns an error if browser initialization fails
    pub fn new() -> Result<Self> {
        Self::with_options(BrowserOptions::default())
    }

    /// Create a new browser service with custom options
    ///
    /// # Errors
    ///
    /// Returns an error if browser initialization fails
    pub fn with_options(options: BrowserOptions) -> Result<Self> {
        Ok(Self {
            browser: Arc::new(RwLock::new(None)),
            options,
        })
    }

    /// Get or create the shared browser instance
    ///
    /// # Errors
    ///
    /// Returns an error if browser launch fails
    async fn get_browser(&self) -> Result<Browser> {
        let mut browser_lock = self.browser.write().await;

        if browser_lock.is_none() {
            info!("Launching headless Chrome browser");

            let launch_options = LaunchOptions::default_builder()
                .headless(self.options.headless)
                .sandbox(self.options.sandbox)
                .window_size(self.options.width, self.options.height)
                .build()
                .context("Failed to build launch options")?;

            let browser = Browser::new(launch_options).context("Failed to launch browser")?;

            *browser_lock = Some(browser);
        }

        Ok(browser_lock.as_ref().unwrap().clone())
    }

    /// Navigate to a URL and capture a page snapshot
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to navigate to
    ///
    /// # Returns
    ///
    /// A `PageSnapshot` containing the page title, content, and screenshot
    ///
    /// # Errors
    ///
    /// Returns an error if navigation or capture fails
    pub async fn navigate(&self, url: &str) -> Result<PageSnapshot> {
        debug!("Navigating to: {}", url);

        let browser = self.get_browser().await?;
        let tab = browser.new_tab().context("Failed to create new tab")?;

        tab.navigate_to(url).context("Failed to navigate")?;

        tab.wait_until_navigated()
            .context("Failed to wait for navigation")?;

        let title = tab.get_title().context("Failed to get page title")?;

        let content = tab.get_content().context("Failed to get page content")?;

        let screenshot = tab
            .capture_screenshot(
                headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
                None,
                None,
                true,
            )
            .context("Failed to capture screenshot")?;

        let url = tab.get_url();

        info!("Successfully captured page: {}", title);

        Ok(PageSnapshot {
            url,
            title,
            content,
            screenshot,
        })
    }

    /// Extract text data from a page using a CSS selector
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to navigate to
    /// * `selector` - The CSS selector to find elements
    ///
    /// # Returns
    ///
    /// The inner text of the first matching element
    ///
    /// # Errors
    ///
    /// Returns an error if navigation fails or element is not found
    pub async fn extract_data(&self, url: &str, selector: &str) -> Result<String> {
        debug!("Extracting data from {} with selector: {}", url, selector);

        let browser = self.get_browser().await?;
        let tab = browser.new_tab().context("Failed to create new tab")?;

        tab.navigate_to(url).context("Failed to navigate")?;

        tab.wait_until_navigated()
            .context("Failed to wait for navigation")?;

        let element = tab.find_element(selector).context(format!(
            "Failed to find element with selector: {}",
            selector
        ))?;

        let text = element
            .get_inner_text()
            .context("Failed to get element text")?;

        debug!("Extracted {} characters", text.len());

        Ok(text)
    }

    /// Click an element on a page
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to navigate to
    /// * `selector` - The CSS selector for the element to click
    ///
    /// # Errors
    ///
    /// Returns an error if navigation fails or element is not found
    pub async fn click_element(&self, url: &str, selector: &str) -> Result<()> {
        debug!("Clicking element {} on {}", selector, url);

        let browser = self.get_browser().await?;
        let tab = browser.new_tab().context("Failed to create new tab")?;

        tab.navigate_to(url).context("Failed to navigate")?;

        tab.wait_until_navigated()
            .context("Failed to wait for navigation")?;

        let element = tab.find_element(selector).context(format!(
            "Failed to find element with selector: {}",
            selector
        ))?;

        element.click().context("Failed to click element")?;

        info!("Successfully clicked element: {}", selector);

        Ok(())
    }

    /// Fill form fields on a page
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to navigate to
    /// * `fields` - Vector of form fields with selectors and values
    ///
    /// # Errors
    ///
    /// Returns an error if navigation fails or any field is not found
    pub async fn fill_form(&self, url: &str, fields: &[FormField]) -> Result<()> {
        debug!("Filling {} form fields on {}", fields.len(), url);

        let browser = self.get_browser().await?;
        let tab = browser.new_tab().context("Failed to create new tab")?;

        tab.navigate_to(url).context("Failed to navigate")?;

        tab.wait_until_navigated()
            .context("Failed to wait for navigation")?;

        for field in fields {
            let element = tab
                .find_element(&field.selector)
                .context(format!("Failed to find form field: {}", field.selector))?;

            element
                .type_into(&field.value)
                .context(format!("Failed to type into field: {}", field.selector))?;

            debug!("Filled field: {}", field.selector);
        }

        info!("Successfully filled {} form fields", fields.len());

        Ok(())
    }

    /// Shutdown the browser instance
    pub async fn shutdown(&self) -> Result<()> {
        let mut browser_lock = self.browser.write().await;
        if let Some(browser) = browser_lock.take() {
            info!("Shutting down browser");
            drop(browser);
        }
        Ok(())
    }
}

/// Browser configuration options
#[derive(Debug, Clone)]
pub struct BrowserOptions {
    pub headless: bool,
    pub sandbox: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for BrowserOptions {
    fn default() -> Self {
        Self {
            headless: true,
            sandbox: true,
            width: Some(1920),
            height: Some(1080),
        }
    }
}

/// Snapshot of a web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSnapshot {
    /// Current URL after navigation
    pub url: String,
    /// Page title
    pub title: String,
    /// Full HTML content
    pub content: String,
    /// PNG screenshot data (base64 encoded for JSON transport)
    pub screenshot: Vec<u8>,
}

/// Form field to fill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// CSS selector for the field
    pub selector: String,
    /// Value to enter
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_options_default() {
        let options = BrowserOptions::default();
        assert!(options.headless);
        assert!(options.sandbox);
        assert_eq!(options.width, Some(1920));
        assert_eq!(options.height, Some(1080));
    }

    #[test]
    fn test_form_field_creation() {
        let field = FormField {
            selector: "#email".to_string(),
            value: "test@example.com".to_string(),
        };
        assert_eq!(field.selector, "#email");
        assert_eq!(field.value, "test@example.com");
    }

    #[tokio::test]
    async fn test_browser_service_creation() {
        let service = BrowserService::new();
        assert!(service.is_ok());
    }
}
