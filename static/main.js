// This file is appended at the end so that all DOM elements should have loaded

document.querySelectorAll('h2.subtitle').forEach(function(subtitle) {
  subtitle.addEventListener('click', function() {
    subtitle.parentElement.classList.toggle('hide');
  });
});
