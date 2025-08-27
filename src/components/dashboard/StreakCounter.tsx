import React from 'react';
import { Skeleton } from '#/components/ui/skeleton';

interface StreakCounterProps {
  current: number | null;
  longest: number | null;
  metricName: string;
  loading?: boolean;
}

export const StreakCounter: React.FC<StreakCounterProps> = ({ 
  current, 
  longest, 
  metricName, 
  loading = false 
}) => {
  if (loading) {
    return <Skeleton className="h-32 w-full rounded-lg" />;
  }

  // Don't render if we don't have streak data
  if (current === null || longest === null) {
    return null;
  }

  return (
    <div className="bg-gradient-to-r from-primary to-accent rounded-lg p-6 text-white">
      <h3 className="text-lg font-semibold mb-4">{metricName} Streak</h3>
      
      <div className="flex items-center justify-between">
        <div className="text-center">
          <div className="text-3xl font-bold mb-1">{current}</div>
          <div className="text-sm opacity-90">Current</div>
        </div>
        
        <div className="text-4xl">🔥</div>
        
        <div className="text-center">
          <div className="text-3xl font-bold mb-1">{longest}</div>
          <div className="text-sm opacity-90">Best</div>
        </div>
      </div>
      
      {current === longest && current > 0 && (
        <div className="mt-4 text-center">
          <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-white bg-opacity-20">
            🏆 Personal Best!
          </span>
        </div>
      )}
    </div>
  );
};