<script>
  import { goto } from "$app/navigation";
	import { onMount } from "svelte";
		
	let data = {};

	onMount(async () => {
		const res = await fetch("/api/user", {
			credentials: "include",
		});
		if (res.status === 401) {
			goto('/login');
		}
		data = await res.json();
	})
</script>

<h1 class="text-4xl">{data.email}</h1>
