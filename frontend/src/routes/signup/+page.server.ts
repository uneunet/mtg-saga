import { redirect } from "@sveltejs/kit";

export const actions = {
	default: async ({ request, fetch }) => {
		const data = await request.formData();
		const name = data.get('name');
		const user_id = data.get('user_id');
		const email = data.get('email');
		const password = data.get("password");

		console.log(data);

		const res = await fetch('http://localhost:3000/api/auth/signup', {
			method: 'POST',
			headers: {
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				"name": name,
				"user_id": user_id,
				"email": email,
				"password": password,
			})
		});
		console.log(res)

		if (res.ok) {
			redirect(302, '/login');
		}
	}
}
