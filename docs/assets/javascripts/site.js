document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll("a[href^='http']").forEach((link) => {
    if (link.hostname !== window.location.hostname) {
      link.rel = "noopener noreferrer";
    }
  });
});

