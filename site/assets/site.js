// Progressive enhancement only. Every page works with this file blocked.
(function () {
  var root = document.documentElement;

  // Links people already have (an email, a résumé) point at the old one-page site's
  // anchors. Send them to where that section lives now.
  var moved = {
    '#projects': 'work/', '#founder': 'work/nisaba/', '#experience': 'about/#experience',
    '#documents': 'resume/', '#contact': 'about/#contact', '#about': 'about/'
  };
  if (document.querySelector('.hero') && moved[location.hash]) {
    location.replace(moved[location.hash]);
    return;
  }

  // Theme: follow the system until the reader picks; remember the pick if storage works.
  var btn = document.querySelector('.theme');
  if (btn) {
    btn.addEventListener('click', function () {
      var dark = root.dataset.theme
        ? root.dataset.theme === 'dark'
        : matchMedia('(prefers-color-scheme: dark)').matches;
      var next = dark ? 'light' : 'dark';
      root.dataset.theme = next;
      try { localStorage.setItem('theme', next); } catch (e) {}
    });
  }

  // Build recordings play while on screen, unless the reader prefers less motion.
  // Without this script they are ordinary videos with controls.
  var still = matchMedia('(prefers-reduced-motion: reduce)').matches;
  var vids = document.querySelectorAll('video[data-autoplay]');
  if (!still && 'IntersectionObserver' in window && vids.length) {
    var io = new IntersectionObserver(function (entries) {
      entries.forEach(function (e) {
        if (e.isIntersecting) { e.target.preload = 'auto'; e.target.play().catch(function () {}); }
        else { e.target.pause(); }
      });
    }, { threshold: 0.4 });
    vids.forEach(function (v) { io.observe(v); });
  }

  // Contact form: send in place and say what happened. Without JS it posts normally.
  var form = document.getElementById('contactForm');
  if (form) {
    var status = document.getElementById('formStatus');
    form.addEventListener('submit', function (e) {
      e.preventDefault();
      var send = form.querySelector('button[type=submit]');
      send.disabled = true;
      status.className = 'form-status';
      status.textContent = 'Sending…';
      fetch(form.action, { method: 'POST', body: new FormData(form), headers: { Accept: 'application/json' } })
        .then(function (r) { return r.json(); })
        .then(function (d) {
          if (d.success) { status.textContent = 'Sent. I will reply by email.'; form.reset(); }
          else { status.className = 'form-status err'; status.textContent = (d.message || 'That did not send.') + ' Email ness.aisling@gmail.com instead.'; }
        })
        .catch(function () {
          status.className = 'form-status err';
          status.textContent = 'That did not send. Email ness.aisling@gmail.com instead.';
        })
        .then(function () { send.disabled = false; });
    });
  }
})();
