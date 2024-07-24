<script lang="ts">
    import { onMount } from "svelte"
    import { invoke } from "@tauri-apps/api/core"

    let ttt: string = "Loading..."
    onMount(async (): Promise<void> => {
        ttt = JSON.parse(await invoke("ttt")).origin
    })
</script>

<div>IP: {ttt}</div>
