<script>

  import { goto } from "$app/navigation";
	import { onMount } from "svelte";
		
	let email = "";

	onMount(async () => {
		const res = await fetch(`/api/user/${data.params.name}`, {
			credentials: "include",
		});
		if (res.status === 401) {
			goto('/login');
		}
		const data = await res.json();
		email = data.email;
	})


	export const data;
</script>

<h1 class="text-4xl">{email}</h1>
