import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'accent' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  children: React.ReactNode;
}

export const Button: React.FC<ButtonProps> = ({ 
  variant = 'primary', 
  size = 'md', 
  children, 
  className = '',
  ...props 
}) => {
  const baseClasses = 'font-medium rounded-lg transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-offset-2';
  
  const variantClasses = {
    primary: 'bg-[var(--color-primary)] text-[var(--color-primary-foreground)] hover:scale-105 active:scale-95 focus:ring-[var(--color-primary)]',
    secondary: 'bg-[var(--color-secondary)] text-[var(--color-secondary-foreground)] hover:scale-105 active:scale-95 focus:ring-[var(--color-secondary)]',
    accent: 'bg-[var(--color-accent)] text-[var(--color-accent-foreground)] hover:scale-105 active:scale-95 focus:ring-[var(--color-accent)]',
    ghost: 'bg-transparent text-[var(--color-foreground)] hover:bg-[var(--color-muted)] active:scale-95'
  };
  
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  };
  
  return (
    <button
      className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
};