import React from 'react';
import { HeatmapDataPoint } from '../../types/metrics';
import { Card } from '../shared/Card';
import { generateDateRange } from '../../utils/dateUtils';

interface CalendarHeatmapProps {
  data: HeatmapDataPoint[];
  title: string;
  startDate: string;
  endDate: string;
}

export const CalendarHeatmap: React.FC<CalendarHeatmapProps> = ({ 
  data, 
  title, 
  startDate, 
  endDate 
}) => {
  const dateRange = generateDateRange(startDate, endDate);
  const dataMap = new Map(data.map(d => [d.date, d]));
  
  // Get intensity color based on level
  const getIntensityColor = (level: number): string => {
    switch (level) {
      case 0: return 'var(--color-muted)';
      case 1: return 'rgba(var(--color-chart-2), 0.3)';
      case 2: return 'rgba(var(--color-chart-2), 0.5)';
      case 3: return 'rgba(var(--color-chart-2), 0.7)';
      case 4: return 'var(--color-chart-2)';
      default: return 'var(--color-muted)';
    }
  };

  // Group dates by weeks
  const weeks: string[][] = [];
  let currentWeek: string[] = [];
  
  dateRange.forEach((date, index) => {
    const dayOfWeek = new Date(date).getDay();
    
    if (index === 0) {
      // Fill empty days at the beginning of first week
      for (let i = 0; i < dayOfWeek; i++) {
        currentWeek.push('');
      }
    }
    
    currentWeek.push(date);
    
    if (dayOfWeek === 6 || index === dateRange.length - 1) {
      // End of week or last date
      if (index === dateRange.length - 1) {
        // Fill empty days at the end of last week
        while (currentWeek.length < 7) {
          currentWeek.push('');
        }
      }
      weeks.push([...currentWeek]);
      currentWeek = [];
    }
  });

  const monthLabels = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 
                      'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  const dayLabels = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  return (
    <Card>
      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-2">{title}</h3>
        <p className="text-sm text-[var(--color-muted-foreground)]">
          Activity from {new Date(startDate).toLocaleDateString()} to {new Date(endDate).toLocaleDateString()}
        </p>
      </div>

      <div className="overflow-x-auto">
        <div className="inline-flex flex-col space-y-1 min-w-max">
          {/* Day labels */}
          <div className="flex space-x-1">
            <div className="w-8"></div>
            {dayLabels.map((day, index) => (
              <div key={index} className="w-3 h-3 text-xs text-[var(--color-muted-foreground)] flex items-center justify-center">
                {index % 2 === 0 ? day[0] : ''}
              </div>
            ))}
          </div>

          {/* Calendar grid */}
          {weeks.map((week, weekIndex) => (
            <div key={weekIndex} className="flex space-x-1 items-center">
              {/* Month label */}
              <div className="w-8 text-xs text-[var(--color-muted-foreground)]">
                {weekIndex === 0 && week[0] ? 
                  monthLabels[new Date(week.find(d => d) || '').getMonth()] : ''}
              </div>
              
              {/* Days in week */}
              {week.map((date, dayIndex) => {
                if (!date) {
                  return <div key={dayIndex} className="w-3 h-3"></div>;
                }
                
                const dayData = dataMap.get(date);
                const level = dayData?.level || 0;
                const count = dayData?.count || 0;
                
                return (
                  <div
                    key={date}
                    className="w-3 h-3 rounded-sm border border-[var(--color-border)] cursor-pointer hover:scale-110 transition-all duration-200"
                    style={{ backgroundColor: getIntensityColor(level) }}
                    title={`${date}: ${count} activities`}
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>

      {/* Legend */}
      <div className="mt-6 flex items-center justify-between">
        <div className="text-sm text-[var(--color-muted-foreground)]">
          Less
        </div>
        <div className="flex space-x-1">
          {[0, 1, 2, 3, 4].map(level => (
            <div
              key={level}
              className="w-3 h-3 rounded-sm border border-[var(--color-border)]"
              style={{ backgroundColor: getIntensityColor(level) }}
            />
          ))}
        </div>
        <div className="text-sm text-[var(--color-muted-foreground)]">
          More
        </div>
      </div>
    </Card>
  );
};