/** Unlicense — public domain — cochranblock.org */
const CACHE = 'rogue-repo-v5';
self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll([
    '/',
    '/manifest.json',
    '/apps/rogue-runner',
    '/apps/rogue-runner-wasm',
    '/login',
    '/register',
    '/assets/apps/rogue-runner.png',
    '/assets/apps/rogue-runner-wasm/rogue-runner.wasm',
    '/assets/apps/rogue-runner-wasm/index.html',
    '/assets/apps/rogue-runner-wasm/zones/00/bg.png',
    '/assets/apps/rogue-runner-wasm/zones/00/ground.png',
    '/assets/apps/rogue-runner-wasm/zones/00/obstacles.png',
    '/assets/apps/rogue-runner-wasm/player/run.png',
    '/assets/apps/rogue-runner-wasm/player/jump.png'
  ])));
});
self.addEventListener('fetch', (e) => {
  const url = e.request.url;
  const isRunnerAsset = url.includes('/assets/apps/rogue-runner-wasm/zones/') ||
    url.includes('/assets/apps/rogue-runner-wasm/player/');
  e.respondWith(caches.match(e.request).then((r) => {
    if (r) return r;
    return fetch(e.request).then((res) => {
      if (isRunnerAsset && res.ok) {
        const clone = res.clone();
        caches.open(CACHE).then((c) => c.put(e.request, clone));
      }
      return res;
    });
  }));
});
