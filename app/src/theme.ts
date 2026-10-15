export function switchTheme(themeName: string) {
  const themeLink = document.getElementById("app-theme") as HTMLLinkElement;

  if (themeLink) {
    themeLink.href = `https://unpkg.com/@mantle-ui/react@10.12.0/resources/themes/${themeName}/theme.css`;
  }
}
