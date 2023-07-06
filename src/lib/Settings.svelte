<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let email = ""
  let msg = ""

    type Config = {
        name: string;
        email: string;
    }

    type Settings = {
        config: null | Config
    }

  async function save(){
    console.log({ name, email })
    msg = await invoke("save", { name, email })
  }

  async function load(){
    msg = "";
    let settings: Settings = await invoke("load")
    if(settings.config) {
        name = settings.config.name;
        email = settings.config.email;
    }
  }

  function handleClear(){
    name = "";
    email = "";
    msg = "";
  }

  async function handleLoad() {
    await load();
  }

  load();
</script>

<div class="column">
  <form on:submit|preventDefault={save}>
	<div class="container" >
		<input id="name-input" placeholder="Enter your name..." bind:value={name} class="row" />
		<input id="email-input" placeholder="Enter your email address..." bind:value={email} class="row" />

		<button type="submit" class="row" style="margin-top: 12px;">Save</button>
	</div>
  </form>
  <button on:click={handleClear} style="margin-top: 12px;">Clear</button>
  <button on:click={handleLoad} style="margin-top: 12px;">Load</button>
  <p>{msg}</p>
</div>
