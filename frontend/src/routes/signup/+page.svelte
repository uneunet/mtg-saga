<script>
  import { goto } from "$app/navigation";

	let name = '';
	let email = '';
	let password = '';
	
	async function handleClick() {
		const res = await fetch('/api/auth/signup', {
			method: 'POST',
			headers: {
    		"Content-Type": "application/json"
 			},
			body: JSON.stringify({
				"name": name,
				"email": email,
				"password": password,
			})
		})

		if (res.ok) {
			alert("Signup Complete! Login to start your saga.");
			goto('/login');
		}
	}
</script>

<input bind:value={name} type="text" placeholder="Name" class="input w-full max-w-xs" />
<input bind:value={email} type="text" placeholder="Email" class="input w-full max-w-xs" />
<input bind:value={password} type="password" placeholder="Password" class="input w-full max-w-xs" />

<button on:click={handleClick} class="btn">Sign Up</button>

