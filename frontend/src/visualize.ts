export function highlightNode(
  dot: string,
  nodeName: string,
  url = "https://www.google.com"
): string {
  const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const nodeStartRegex = new RegExp(`\\b${esc(nodeName)}\\s*\\[`, "g");

  const inserts: { closeIndex: number }[] = [];
  let m: RegExpExecArray | null;

  // helper to detect if a position is inside quotes
  const isInsideQuotes = (s: string, pos: number) => {
    let inDouble = false;
    let inSingle = false;
    let escaped = false;
    for (let i = 0; i < pos; i++) {
      const ch = s[i];
      if (escaped) {
        escaped = false;
        continue;
      }
      if (ch === "\\") {
        escaped = true;
        continue;
      }
      if (ch === '"' && !inSingle) inDouble = !inDouble;
      else if (ch === "'" && !inDouble) inSingle = !inSingle;
    }
    return inDouble || inSingle;
  };

  while ((m = nodeStartRegex.exec(dot)) !== null) {
    const matchPos = m.index;
    const openBracketIndex = nodeStartRegex.lastIndex - 1; // position of '['

    // skip if this occurrence is inside quotes (e.g. inside label)
    if (isInsideQuotes(dot, matchPos)) continue;

    // find matching closing ']' that is not inside quotes
    let inDouble = false;
    let inSingle = false;
    let escaped = false;
    let closeIndex = -1;
    for (let i = openBracketIndex + 1; i < dot.length; i++) {
      const ch = dot[i];
      if (escaped) {
        escaped = false;
        continue;
      }
      if (ch === "\\") {
        escaped = true;
        continue;
      }
      if (ch === '"' && !inSingle) {
        inDouble = !inDouble;
        continue;
      }
      if (ch === "'" && !inDouble) {
        inSingle = !inSingle;
        continue;
      }
      if (ch === "]" && !inDouble && !inSingle) {
        closeIndex = i;
        break;
      }
    }
    if (closeIndex === -1) continue; // malformed, skip

    // check if attrs already contain styling
    const attrs = dot.slice(openBracketIndex + 1, closeIndex);
    if (/(?:\bcolor\b|\bfillcolor\b|\bstyle\b)/i.test(attrs)) {
      // already highlighted or styled — skip
      nodeStartRegex.lastIndex = closeIndex + 1;
      continue;
    }

    inserts.push({ closeIndex });
    nodeStartRegex.lastIndex = closeIndex + 1; // continue after this node
  }

  if (inserts.length === 0) return dot;

  // build new string with insertions (left-to-right)
  let out = "";
  let last = 0;
  const insertStr = `, color=green, URL="${url}", target="_blank",  fillcolor=green, fontcolor=green`;
  for (const ins of inserts) {
    out += dot.slice(last, ins.closeIndex) + insertStr + "]";
    last = ins.closeIndex + 1;
  }
  out += dot.slice(last);
  return out;
}

function prefixSpecificNode(
  dot: string,
  nodeName: string,
  prefix: string
): string {
  // Regex para el nodo específico con label sin comillas
  const nodeRegex = new RegExp(`(${nodeName}\\s*\\[label=\\{)([^}]*)\\}`, "g");

  return dot.replace(nodeRegex, (match, start, content) => {
    // solo agregamos el prefijo al primer bloque (antes del primer pipe)
    const parts = content.split("|");
    parts[0] = `(${prefix}) ${parts[0].trim()}`;
    const newLabel = parts.join(" | ");
    return `${start}${newLabel}}`;
  });
}
