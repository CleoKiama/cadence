import React from 'react';

interface CardProps {
  children: React.ReactNode;
  className?: string;
  variant?: 'default' | 'elevated';
  hoverable?: boolean;
}

export const Card: React.FC<CardProps> = ({ 
  children, 
  className = '', 
  variant = 'default',
  hoverable = false 
}) => {
  const baseClasses = 'rounded-lg border p-6 bg-[var(--color-card)] text-[var(--color-card-foreground)] border-[var(--color-border)]';
  const variantClasses = variant === 'elevated' ? 'shadow-lg' : 'shadow-sm';
  const hoverClasses = hoverable ? 'transition-all duration-200 hover:shadow-lg hover:-translate-y-1' : '';
  
  return (
    <div className={`${baseClasses} ${variantClasses} ${hoverClasses} ${className}`}>
      {children}
    </div>
  );
};