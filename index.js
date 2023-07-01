
const url = new URL(document.location.href);
const params = url.searchParams;
const id = params.get('id') ?? 1;

import(`./crates/nannou_${id}/pkg/nannou_${id}.js`).then(
  async ({default: nannou}) => {
    await nannou().then((s) => {
      s.main_web();
    });
  },
);
