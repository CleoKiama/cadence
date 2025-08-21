import React from 'react';
import { ThemeToggle } from '../shared/ThemeToggle';

interface HeaderProps {
  title?: string;
}

export const Header: React.FC<HeaderProps> = ({ title = 'Habitron' }) => {
  return (
    <header className="border-b border-border bg-background">
      <div className="flex items-center justify-between px-6 py-4">
        <div className="flex items-center space-x-4">
          <div className="text-2xl font-bold text-foreground">
            {title}
          </div>
          <div className="hidden sm:block text-sm text-muted-foreground">
            Track your habits, build your future
          </div>
        </div>
        
        <div className="flex items-center space-x-4">
          <div className="text-sm text-muted-foreground">
            {new Date().toLocaleDateString('en-US', { 
              weekday: 'long', 
              year: 'numeric', 
              month: 'short', 
              day: 'numeric' 
            })}
          </div>
          <ThemeToggle />
        </div>
      </div>
    </header>
  );
};