import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = async ({ fetch }): PageServerLoad => {
	const res = await fetch('http://localhost:3000/api/user');
	if (res.status === 401) {
		throw redirect(302, '/login');
	}
	const data = await res.json();

	return data;
}

