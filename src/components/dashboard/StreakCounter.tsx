import React from 'react';

interface StreakCounterProps {
  current: number;
  longest: number;
  metricName: string;
}

export const StreakCounter: React.FC<StreakCounterProps> = ({ current, longest, metricName }) => {
  return (
    <div className="bg-gradient-to-r from-[var(--color-primary)] to-[var(--color-accent)] rounded-lg p-6 text-white">
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