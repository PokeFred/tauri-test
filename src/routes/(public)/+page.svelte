<script lang="ts">
    import { invoke } from "@tauri-apps/api/core"

    setInterval(async (): Promise<void> => {
        await invoke<void>("set_app_title", { title: (new Date()) })
    }, 1000)
</script>

<div>Hello World</div>
