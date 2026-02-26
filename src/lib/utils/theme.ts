export type AppTheme = 'dark' | 'light';

export function normalizeTheme(theme: string | null | undefined): AppTheme {
  return theme === 'light' ? 'light' : 'dark';
}

export function applyTheme(theme: string | null | undefined): AppTheme {
  const normalizedTheme = normalizeTheme(theme);
  document.documentElement.setAttribute('data-theme', normalizedTheme);
  return normalizedTheme;
}
