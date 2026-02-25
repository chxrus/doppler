const MODIFIER_TOKENS = new Set(['CommandOrControl', 'Shift', 'Alt']);

function normalizeToken(token: string): string {
  if (token === 'Ctrl' || token === 'Control' || token === 'Command') {
    return 'CommandOrControl';
  }
  return token;
}

function keyTokenFromCode(code: string): string | null {
  if (code.startsWith('Key') && code.length === 4) {
    return code.slice(3).toUpperCase();
  }
  if (code.startsWith('Digit') && code.length === 6) {
    return code.slice(5);
  }

  const mapped: Record<string, string> = {
    Comma: ',',
    Period: '.',
    Slash: '/',
    Semicolon: ';',
    Quote: "'",
    BracketLeft: '[',
    BracketRight: ']',
    Minus: '-',
    Equal: '=',
    Backquote: '`',
    Backslash: '\\',
    Space: 'Space',
    Enter: 'Enter',
    Escape: 'Escape',
    Tab: 'Tab',
    Backspace: 'Backspace',
    Delete: 'Delete',
    ArrowLeft: 'Left',
    ArrowRight: 'Right',
    ArrowUp: 'Up',
    ArrowDown: 'Down'
  };

  if (mapped[code] != null) {
    return mapped[code];
  }

  if (code.startsWith('F')) {
    return code;
  }

  return null;
}

function parseHotkey(hotkey: string): {
  needsPrimary: boolean;
  needsShift: boolean;
  needsAlt: boolean;
  key: string | null;
} {
  const tokens = hotkey
    .split('+')
    .map(token => normalizeToken(token.trim()))
    .filter(token => token.length > 0);

  let key: string | null = null;
  for (const token of tokens) {
    if (!MODIFIER_TOKENS.has(token)) {
      key = token;
      break;
    }
  }

  return {
    needsPrimary: tokens.includes('CommandOrControl'),
    needsShift: tokens.includes('Shift'),
    needsAlt: tokens.includes('Alt'),
    key
  };
}

export function isHotkeyPressed(event: KeyboardEvent, hotkey: string): boolean {
  const parsed = parseHotkey(hotkey);
  if (parsed.key == null) {
    return false;
  }

  const hasPrimary = event.metaKey || event.ctrlKey;
  if (hasPrimary !== parsed.needsPrimary) return false;
  if (event.shiftKey !== parsed.needsShift) return false;
  if (event.altKey !== parsed.needsAlt) return false;

  const eventKeyToken = keyTokenFromCode(event.code);
  return eventKeyToken === parsed.key;
}

export function buildHotkeyFromEvent(event: KeyboardEvent): string | null {
  const keyToken = keyTokenFromCode(event.code);
  if (keyToken == null) {
    return null;
  }

  const isModifierOnly =
    event.code === 'ShiftLeft' ||
    event.code === 'ShiftRight' ||
    event.code === 'ControlLeft' ||
    event.code === 'ControlRight' ||
    event.code === 'MetaLeft' ||
    event.code === 'MetaRight' ||
    event.code === 'AltLeft' ||
    event.code === 'AltRight';
  if (isModifierOnly) {
    return null;
  }

  const parts: string[] = [];
  if (event.metaKey || event.ctrlKey) {
    parts.push('CommandOrControl');
  }
  if (event.shiftKey) {
    parts.push('Shift');
  }
  if (event.altKey) {
    parts.push('Alt');
  }
  parts.push(keyToken);

  return parts.join('+');
}

export function formatHotkeyLabel(hotkey: string): string {
  return hotkey
    .split('+')
    .map((token) => {
      const trimmed = token.trim();
      if (trimmed === 'CommandOrControl' || trimmed === 'Command' || trimmed === 'Control' || trimmed === 'Ctrl') {
        return 'cmd/^';
      }
      return trimmed;
    })
    .join('+');
}
