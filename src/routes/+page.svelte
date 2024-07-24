<script lang="ts">
    import { onMount } from "svelte"
    import { invoke } from "@tauri-apps/api/core"

    let fetchTestOne: string = "Loading..."
    let fetchTestTwo: string = "Loading..."
    let fetchTestThree: string = "Loading..."

    async function update() {
        fetchTestOne = "Loading..."
        fetchTestTwo = "Loading..."
        fetchTestThree = "Loading..."

        invoke<string>("fetch_test_one").then((value: string): void => {
            fetchTestOne = value
        })

        invoke<string>("fetch_test_two").then((value: string): void => {
            fetchTestTwo = value
        })

        invoke<string>("fetch_test_three").then((value: string): void => {
            fetchTestThree = value
        })
    }

    onMount(update)
</script>

<button on:click={update}>Reload</button>
<div>{fetchTestOne}</div>
<div>{fetchTestTwo}</div>
<div>{fetchTestThree}</div>
