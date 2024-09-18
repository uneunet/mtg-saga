import { redirect } from "@sveltejs/kit";

export const actions = {
	default: async ({ request, fetch }) => {
		const data = await request.formData();
		const name = data.get('name');
		const email = data.get('email');
		const password = data.password("password");

		const res = await fetch('http://localhost:3000/api/auth/signup', {
			method: 'POST',
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				"name": name,
				"email": email,
				"password": password,
			})
		});

		if (res.ok) {
			redirect(302, '/login');
		}
	}
}
