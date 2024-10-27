import Elysia from 'elysia';

type Social =
	| 'github'
	| 'instagram'
	| 'soundcloud'
	| 'spotify'
	| 'telegram'
	| 'vk'
	| 'pypi'
	| 'npmjs';

const socials: Record<Social, string> = {
	github: 'https://github.com/moose23967',
	instagram: 'https://instagram.com/musu23967',
	soundcloud: 'https://on.soundcloud.com/MXHoANuGsfFwbQys7',
	spotify: 'https://open.spotify.com/user/31jgugynqmzpiwoxdkjqor6degdu',
	telegram: 'https://t.me/moose23967',
	vk: 'https://vk.com/musu23967',
	pypi: 'https://pypi.org/user/moose23967',
	npmjs: 'https://npmjs.com/~musu23967',
};

new Elysia({
	name: 'moose23967-ru',
})
	.get('/', ({ redirect }) => redirect('https://campsite.bio/moose23967'))
	.get('/socials/:social', ({ redirect, params: { social } }) => {
		if (social in socials) {
			return redirect(socials[social as Social]);
		}

		return 'Meow';
	})
	.get('/uptime', () => 'Meow')
	.listen(10_000);
