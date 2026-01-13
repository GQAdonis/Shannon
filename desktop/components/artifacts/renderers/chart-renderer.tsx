/**
 * Chart Renderer using Recharts
 *
 * Renders interactive charts with multiple visualization types
 */

'use client';

import { useState } from 'react';
import {
  LineChart,
  Line,
  BarChart,
  Bar,
  PieChart,
  Pie,
  AreaChart,
  Area,
  ScatterChart,
  Scatter,
  RadarChart,
  Radar,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  Cell,
} from 'recharts';
import { Artifact, ChartConfig } from '@/lib/artifacts/types';
import { Button } from '@/components/ui/button';
import { Download, AlertCircle } from 'lucide-react';
import { toast } from 'sonner';

interface ChartRendererProps {
  artifact: Artifact;
}

const COLORS = ['#8884d8', '#82ca9d', '#ffc658', '#ff7c7c', '#8dd1e1', '#d084d0'];

export function ChartRenderer({ artifact }: ChartRendererProps) {
  const [error, setError] = useState<string | null>(null);

  let chartConfig: ChartConfig;
  try {
    chartConfig = JSON.parse(artifact.content) as ChartConfig;
  } catch (err) {
    return (
      <div className="chart-renderer border rounded-lg p-4">
        <div className="flex items-start gap-3 p-4 bg-destructive/10 text-destructive rounded-md">
          <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
          <div>
            <p className="font-medium">Invalid chart configuration</p>
            <p className="text-sm mt-1">Failed to parse chart data</p>
          </div>
        </div>
      </div>
    );
  }

  const handleExport = () => {
    const dataStr = JSON.stringify(chartConfig, null, 2);
    const blob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `${artifact.title.replace(/\s+/g, '-')}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    toast.success('Chart data exported');
  };

  const renderChart = () => {
    const { type, data, xAxisKey, yAxisKey, colors = COLORS, legend = true, grid = true } = chartConfig;

    const commonProps = {
      data,
      margin: { top: 5, right: 30, left: 20, bottom: 5 },
    };

    switch (type) {
      case 'line':
        return (
          <LineChart {...commonProps}>
            {grid && <CartesianGrid strokeDasharray="3 3" />}
            <XAxis dataKey={xAxisKey || 'name'} />
            <YAxis />
            <Tooltip />
            {legend && <Legend />}
            <Line type="monotone" dataKey={yAxisKey || 'value'} stroke={colors[0]} />
          </LineChart>
        );

      case 'bar':
        return (
          <BarChart {...commonProps}>
            {grid && <CartesianGrid strokeDasharray="3 3" />}
            <XAxis dataKey={xAxisKey || 'name'} />
            <YAxis />
            <Tooltip />
            {legend && <Legend />}
            <Bar dataKey={yAxisKey || 'value'} fill={colors[0]} />
          </BarChart>
        );

      case 'area':
        return (
          <AreaChart {...commonProps}>
            {grid && <CartesianGrid strokeDasharray="3 3" />}
            <XAxis dataKey={xAxisKey || 'name'} />
            <YAxis />
            <Tooltip />
            {legend && <Legend />}
            <Area type="monotone" dataKey={yAxisKey || 'value'} stroke={colors[0]} fill={colors[0]} />
          </AreaChart>
        );

      case 'pie':
        return (
          <PieChart>
            <Pie
              data={data}
              dataKey={yAxisKey || 'value'}
              nameKey={xAxisKey || 'name'}
              cx="50%"
              cy="50%"
              outerRadius={80}
              label
            >
              {data.map((entry) => {
                const index = data.indexOf(entry);
                return (
                  <Cell
                    key={entry[xAxisKey || 'name'] || `cell-${index}`}
                    fill={colors[index % colors.length]}
                  />
                );
              })}
            </Pie>
            <Tooltip />
            {legend && <Legend />}
          </PieChart>
        );

      case 'scatter':
        return (
          <ScatterChart {...commonProps}>
            {grid && <CartesianGrid strokeDasharray="3 3" />}
            <XAxis dataKey={xAxisKey || 'x'} />
            <YAxis dataKey={yAxisKey || 'y'} />
            <Tooltip />
            {legend && <Legend />}
            <Scatter name="Data" data={data} fill={colors[0]} />
          </ScatterChart>
        );

      case 'radar':
        return (
          <RadarChart cx="50%" cy="50%" outerRadius="80%" data={data}>
            <PolarGrid />
            <PolarAngleAxis dataKey={xAxisKey || 'subject'} />
            <PolarRadiusAxis />
            <Radar name="Data" dataKey={yAxisKey || 'value'} stroke={colors[0]} fill={colors[0]} fillOpacity={0.6} />
            <Tooltip />
            {legend && <Legend />}
          </RadarChart>
        );

      default:
        setError(`Unsupported chart type: ${type}`);
        return null;
    }
  };

  if (error) {
    return (
      <div className="chart-renderer border rounded-lg p-4">
        <div className="flex items-start gap-3 p-4 bg-destructive/10 text-destructive rounded-md">
          <AlertCircle className="h-5 w-5 mt-0.5 flex-shrink-0" />
          <div>
            <p className="font-medium">Chart rendering error</p>
            <p className="text-sm mt-1">{error}</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="chart-renderer border rounded-lg overflow-hidden">
      <div className="bg-muted px-4 py-2 border-b flex items-center justify-between">
        <h4 className="font-medium text-sm">{artifact.title}</h4>
        <div className="flex items-center gap-2">
          <span className="text-xs px-2 py-0.5 bg-primary/10 text-primary rounded">
            {chartConfig.type}
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleExport}
            title="Export chart data"
          >
            <Download className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div className="bg-background p-4">
        <ResponsiveContainer width="100%" height={artifact.metadata.height || 400}>
          {renderChart() || <div />}
        </ResponsiveContainer>
      </div>

      {artifact.metadata.description && (
        <div className="border-t px-4 py-2 text-sm text-muted-foreground">
          {artifact.metadata.description}
        </div>
      )}
    </div>
  );
}
