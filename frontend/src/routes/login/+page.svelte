<script>
  import { goto } from "$app/navigation";

	let email = '';
	let password = '';
	
	let loginFailed = false;

	async function handleClick() {
		const res = await fetch('/api/auth/login', {
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
			localStorage.setItem("jwt", token);
			goto('/profile');
		} else {
			loginFailed = true;
		}
	}
</script>
{#if loginFailed}
	<p>Login Failed!</p>
{/if}
<input bind:value={email} type="text" placeholder="Email" class="input w-full max-w-xs" />
<input bind:value={password} type="password" placeholder="Password" class="input w-full max-w-xs" />

<button on:click={handleClick} class="btn">Login</button>

