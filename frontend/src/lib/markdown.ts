import { marked } from 'marked';
import markedKatex from 'marked-katex-extension';
import sanitizeHtml from 'sanitize-html';

marked.setOptions({ gfm: true, breaks: true });
marked.use(markedKatex({ throwOnError: false, output: 'htmlAndMathml' }));

const mathTags = [
  'math', 'semantics', 'annotation', 'mrow', 'mi', 'mn', 'mo', 'mtext', 'mspace',
  'msup', 'msub', 'msubsup', 'mfrac', 'msqrt', 'mroot', 'mover', 'munder',
  'munderover', 'mtable', 'mtr', 'mtd', 'mpadded', 'mphantom', 'menclose',
  'svg', 'path'
];

export function renderMarkdown(source: string): string {
  const html = marked.parse(source) as string;
  return sanitizeHtml(html, {
    allowedTags: sanitizeHtml.defaults.allowedTags.concat(['h1', 'h2', 'h3', 'h4', 'table', 'thead', 'tbody', 'tr', 'th', 'td', 'img', ...mathTags]),
    allowedAttributes: {
      ...sanitizeHtml.defaults.allowedAttributes,
      a: ['href', 'name', 'target', 'rel'],
      img: ['src', 'alt', 'title', 'width', 'height', 'loading'],
      span: ['class', 'style', 'aria-hidden'],
      math: ['xmlns', 'display'],
      annotation: ['encoding'],
      svg: ['xmlns', 'width', 'height', 'viewBox', 'preserveAspectRatio'],
      path: ['d']
    },
    allowedStyles: {
      span: {
        height: [/^-?[\d.]+(?:em|ex|px|%)?$/],
        width: [/^-?[\d.]+(?:em|ex|px|%)?$/],
        top: [/^-?[\d.]+(?:em|ex|px|%)?$/],
        left: [/^-?[\d.]+(?:em|ex|px|%)?$/],
        'margin-left': [/^-?[\d.]+(?:em|ex|px|%)?$/],
        'margin-right': [/^-?[\d.]+(?:em|ex|px|%)?$/],
        'vertical-align': [/^-?[\d.]+(?:em|ex|px|%)?$/],
        'border-bottom-width': [/^[\d.]+(?:em|ex|px)?$/]
      }
    },
    allowedSchemes: ['http', 'https', 'mailto'],
    allowedSchemesByTag: { img: ['http', 'https'] }
  });
}
