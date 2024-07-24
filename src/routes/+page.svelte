<script lang="ts">
    import { onMount } from "svelte"
    import { invoke } from "@tauri-apps/api/core"

    let authCheck: string = "Loading..."
    async function checkAuth() {
        authCheck = "Loading..."
        authCheck = await invoke("check_auth")
    }

    onMount(checkAuth)
</script>

<button on:click={checkAuth}>Reload</button>
<div>{authCheck}</div>
