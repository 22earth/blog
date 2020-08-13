export function genRandomStr(len: number): string {
  return Array.apply(null, Array(len))
    .map(function () {
      return (function (chars) {
        return chars.charAt(Math.floor(Math.random() * chars.length))
      })('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789')
    })
    .join('')
}

export function randomNum(max: number, min: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min
}
