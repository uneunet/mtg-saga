import { redirect } from "@sveltejs/kit";

export async function load() {
	const token = localStorage.getItem('jwt');

	if (!token) {
		throw redirect(302, 'login');
	}

	const res = await fetch('/api/user', {
		headers: {
			'Authorization': `Bearer ${token}`
		}
	});

	if (!res.ok) {
		throw redirect(302, '/login');
	}

	const user = await res.json();
	return user;
}
