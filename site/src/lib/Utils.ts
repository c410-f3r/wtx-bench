export function dateAndTime(date: Date): string {
  return date.toLocaleString();
}

export function lastIterElem<T>(iter: IterableIterator<T>): T | undefined {
  let rslt = undefined;
  for (const elem of iter) {
    rslt = elem;
  }
  return rslt;
}

export function randomColor() {
  var letters = '0123456789ABCDEF'.split('');
  var color = '#';
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}
