function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function safeHref(url: string): string {
  const trimmedUrl = url.trim();
  if (
    trimmedUrl.startsWith('http://') ||
    trimmedUrl.startsWith('https://') ||
    trimmedUrl.startsWith('mailto:')
  ) {
    return trimmedUrl;
  }
  return '#';
}

function applyInlineMarkdown(value: string): string {
  let result = value;
  const codeTokens: string[] = [];

  result = result.replace(/`([^`]+)`/g, (_match, codeContent: string) => {
    const token = `__CODE_TOKEN_${codeTokens.length}__`;
    codeTokens.push(`<code>${codeContent}</code>`);
    return token;
  });

  result = result.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_match, text: string, href: string) => {
    const safeUrl = safeHref(href);
    return `<a href="${safeUrl}" target="_blank" rel="noopener noreferrer">${text}</a>`;
  });
  result = result.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
  result = result.replace(/\*([^*]+)\*/g, '<em>$1</em>');
  result = result.replace(/__CODE_TOKEN_(\d+)__/g, (_match, index: string) => {
    const tokenIndex = Number(index);
    return codeTokens[tokenIndex] ?? '';
  });

  return result;
}

export function renderMarkdown(markdown: string): string {
  const lines = escapeHtml(markdown).replaceAll('\r\n', '\n').split('\n');
  const htmlParts: string[] = [];
  const paragraphLines: string[] = [];
  let inCodeBlock = false;
  let codeBlockLines: string[] = [];
  let activeListType: 'ul' | 'ol' | null = null;

  const flushParagraph = () => {
    if (paragraphLines.length === 0) return;
    const content = paragraphLines.map((line) => applyInlineMarkdown(line)).join('<br/>');
    htmlParts.push(`<p>${content}</p>`);
    paragraphLines.length = 0;
  };

  const flushList = () => {
    if (activeListType !== null) {
      htmlParts.push(`</${activeListType}>`);
      activeListType = null;
    }
  };

  const openListIfNeeded = (type: 'ul' | 'ol') => {
    if (activeListType === type) return;
    flushParagraph();
    flushList();
    activeListType = type;
    htmlParts.push(`<${type}>`);
  };

  for (const line of lines) {
    const fenceMatch = line.match(/^```/);
    if (fenceMatch) {
      flushParagraph();
      flushList();
      if (inCodeBlock) {
        htmlParts.push(`<pre><code>${codeBlockLines.join('\n')}</code></pre>`);
        codeBlockLines = [];
        inCodeBlock = false;
      } else {
        inCodeBlock = true;
      }
      continue;
    }

    if (inCodeBlock) {
      codeBlockLines.push(line);
      continue;
    }

    if (line.trim() === '') {
      flushParagraph();
      flushList();
      continue;
    }

    const headingMatch = line.match(/^(#{1,6})\s+(.+)$/);
    if (headingMatch) {
      flushParagraph();
      flushList();
      const level = headingMatch[1].length;
      htmlParts.push(`<h${level}>${applyInlineMarkdown(headingMatch[2])}</h${level}>`);
      continue;
    }

    const bulletMatch = line.match(/^[-*+]\s+(.+)$/);
    if (bulletMatch) {
      openListIfNeeded('ul');
      htmlParts.push(`<li>${applyInlineMarkdown(bulletMatch[1])}</li>`);
      continue;
    }

    const orderedMatch = line.match(/^\d+\.\s+(.+)$/);
    if (orderedMatch) {
      openListIfNeeded('ol');
      htmlParts.push(`<li>${applyInlineMarkdown(orderedMatch[1])}</li>`);
      continue;
    }

    const quoteMatch = line.match(/^>\s?(.+)$/);
    if (quoteMatch) {
      flushParagraph();
      flushList();
      htmlParts.push(`<blockquote>${applyInlineMarkdown(quoteMatch[1])}</blockquote>`);
      continue;
    }

    flushList();
    paragraphLines.push(line);
  }

  flushParagraph();
  flushList();

  if (inCodeBlock) {
    htmlParts.push(`<pre><code>${codeBlockLines.join('\n')}</code></pre>`);
  }

  return htmlParts.join('');
}
