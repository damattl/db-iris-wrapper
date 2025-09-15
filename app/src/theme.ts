export function switchTheme(themeName: string) {
  const themeLink = document.getElementById("app-theme") as HTMLLinkElement;

  if (themeLink) {
    themeLink.href = `https://unpkg.com/primereact/resources/themes/${themeName}/theme.css`;
  }
}
