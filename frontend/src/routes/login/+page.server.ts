import { redirect } from "@sveltejs/kit";

export const actions = {
	default: async ({ cookies, request, fetch }) => {
		const data = await request.formData();
		const email = data.get('email');
		const password = data.get('password');

		const res = await fetch('http://localhost:3000/api/auth/login', {
			method: 'POST',
			headers: {
    		"Content-Type": "application/json"
 			},
			body: JSON.stringify({
				"email": email,
				"password": password,
			})
		})
	
		if (res.ok) {
			const token = await res.text();

			cookies.set('jwt', token, {
				path: '/',
				httpOnly: true,
				secure: true,
				sameSite: false,
			});
			throw redirect(302, '/profile');
		}
	}
}
