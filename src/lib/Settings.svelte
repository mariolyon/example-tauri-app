<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let email = ""
  let msg = ""

  async function save(){
    msg = await invoke("save", { name, email })
  }
</script>

<div class="column">
  <form on:submit|preventDefault={save}>
	<div class="container" >
		<input id="name-input" placeholder="Enter your name..." bind:value={name} class="row" />
		<input id="email-input" placeholder="Enter your email address..." bind:value={email} class="row" />
		
		<button type="submit" class="row" style="margin-top: 12px;">Save</button>
	</div>
  </form>
  <p>{msg}</p>
</div>
