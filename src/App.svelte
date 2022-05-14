<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from "svelte";

    export let name: string;
    let container;
    let info: string = 'Empty';
    function call_rust() {
        invoke('cmd_with_return')
    }
    function runCommand(commandName) {
        const result = document.querySelector('#response')
        invoke(commandName).then((response) => {
            result.innerHTML = `Ok(${response})`
        })
            .catch((error) => {
                result.innerHTML = `Err(${error})`
            })
    }
    function build_controls() {
        const button = document.createElement('button')
        button.innerHTML = `Run command`
        button.addEventListener('click', function () {
            runCommand('my_custom_command')
        })
        container.appendChild(button)
        console.log("build_controls called !!!")
    }
    async function test_me() {
        invoke('full_command', {
            number: 42,
        })
            .then((res) => {
                    console.log(`Message: ${res.message}, Other Val: ${res.other_val}`);
                    info = `Message: ${res.message}, Other Val: ${res.other_val}`;
            })
            .catch((e) => console.error(e))
    }
    onMount(() => build_controls())
    invoke('end_of_load')
</script>

<main>
    <h1>Hello {name}!</h1>
    <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
    <p>How you ok with that ?</p>
    <p>Actions</p>
    <div bind:this={container}></div>
    <button on:click={call_rust}>Click to call rust</button>
    <button on:click={test_me}>Full command</button>
    <div id="result">{info}</div>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: 1024px;
        }
    }
</style>