/**
 * Artifact Database Service
 *
 * Client-side storage for artifacts using IndexedDB via Dexie
 */

import Dexie, { type Table } from 'dexie';
import { Artifact, ArtifactFilter } from './types';

export class ArtifactDatabase extends Dexie {
  artifacts!: Table<Artifact, string>;

  constructor() {
    super('ShannonArtifacts');

    this.version(1).stores({
      artifacts: 'id, type, conversationId, messageId, createdAt, updatedAt, *tags',
    });
  }
}

// Singleton instance
export const artifactDb = new ArtifactDatabase();

/**
 * Artifact Service for CRUD operations
 */
export class ArtifactService {
  private db: ArtifactDatabase;

  constructor(db: ArtifactDatabase = artifactDb) {
    this.db = db;
  }

  /**
   * Save artifact to database
   */
  async save(artifact: Artifact): Promise<string> {
    await this.db.artifacts.put(artifact);
    return artifact.id;
  }

  /**
   * Save multiple artifacts
   */
  async saveMany(artifacts: Artifact[]): Promise<string[]> {
    await this.db.artifacts.bulkPut(artifacts);
    return artifacts.map(a => a.id);
  }

  /**
   * Get artifact by ID
   */
  async get(id: string): Promise<Artifact | undefined> {
    return this.db.artifacts.get(id);
  }

  /**
   * List artifacts with optional filtering
   */
  async list(filter: ArtifactFilter = {}): Promise<Artifact[]> {
    let query = this.db.artifacts.toCollection();

    // Filter by type
    if (filter.type) {
      const types = Array.isArray(filter.type) ? filter.type : [filter.type];
      query = query.filter(a => types.includes(a.type));
    }

    // Filter by conversation
    if (filter.conversationId) {
      query = query.filter(a => a.conversationId === filter.conversationId);
    }

    // Filter by message
    if (filter.messageId) {
      query = query.filter(a => a.messageId === filter.messageId);
    }

    // Filter by date range
    if (filter.dateFrom) {
      query = query.filter(a => a.createdAt >= filter.dateFrom!);
    }
    if (filter.dateTo) {
      query = query.filter(a => a.createdAt <= filter.dateTo!);
    }

    // Filter by tags
    if (filter.tags && filter.tags.length > 0) {
      query = query.filter(a =>
        filter.tags!.some(tag => a.metadata.tags?.includes(tag))
      );
    }

    // Text search
    if (filter.search) {
      const searchLower = filter.search.toLowerCase();
      query = query.filter(a =>
        a.title.toLowerCase().includes(searchLower) ||
        a.content.toLowerCase().includes(searchLower)
      );
    }

    // Sort by creation date (newest first)
    return query.reverse().sortBy('createdAt');
  }

  /**
   * Delete artifact by ID
   */
  async delete(id: string): Promise<void> {
    await this.db.artifacts.delete(id);
  }

  /**
   * Delete multiple artifacts
   */
  async deleteMany(ids: string[]): Promise<void> {
    await this.db.artifacts.bulkDelete(ids);
  }

  /**
   * Search artifacts by text
   */
  async search(query: string): Promise<Artifact[]> {
    return this.list({ search: query });
  }

  /**
   * Get artifacts by conversation
   */
  async getByConversation(conversationId: string): Promise<Artifact[]> {
    return this.db.artifacts
      .where('conversationId')
      .equals(conversationId)
      .reverse()
      .sortBy('createdAt');
  }

  /**
   * Get artifacts by message
   */
  async getByMessage(messageId: string): Promise<Artifact[]> {
    return this.db.artifacts
      .where('messageId')
      .equals(messageId)
      .toArray();
  }

  /**
   * Update artifact
   */
  async update(id: string, updates: Partial<Artifact>): Promise<void> {
    const artifact = await this.get(id);
    if (!artifact) {
      throw new Error(`Artifact ${id} not found`);
    }

    await this.db.artifacts.update(id, {
      ...updates,
      updatedAt: new Date().toISOString(),
    });
  }

  /**
   * Get artifact count
   */
  async count(filter: ArtifactFilter = {}): Promise<number> {
    const artifacts = await this.list(filter);
    return artifacts.length;
  }

  /**
   * Get artifact statistics
   */
  async getStats(): Promise<{
    total: number;
    byType: Record<string, number>;
    recent: Artifact[];
  }> {
    const all = await this.db.artifacts.toArray();

    const byType: Record<string, number> = {};
    for (const artifact of all) {
      byType[artifact.type] = (byType[artifact.type] || 0) + 1;
    }

    const recent = await this.db.artifacts
      .orderBy('createdAt')
      .reverse()
      .limit(10)
      .toArray();

    return {
      total: all.length,
      byType,
      recent,
    };
  }

  /**
   * Clear all artifacts
   */
  async clear(): Promise<void> {
    await this.db.artifacts.clear();
  }

  /**
   * Export artifacts to JSON
   */
  async export(): Promise<string> {
    const artifacts = await this.db.artifacts.toArray();
    return JSON.stringify(artifacts, null, 2);
  }

  /**
   * Import artifacts from JSON
   */
  async import(json: string): Promise<number> {
    const artifacts = JSON.parse(json) as Artifact[];
    await this.db.artifacts.bulkPut(artifacts);
    return artifacts.length;
  }
}

// Singleton service instance
export const artifactService = new ArtifactService();
