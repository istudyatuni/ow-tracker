<script>
    import { export_save_to_browser_url, get_save_opened_facts } from "../lib/saves";

  let opened = $state(false);
  let input;

  function toggle_open() {
    opened = !opened;
  }
  function on_file_upload_click() {
    input.click();
  }
  async function handle_file_change(e) {
    let file = await e.target.files[0].text();
    let data = JSON.parse(file).shipLogFactSaves
    export_save_to_browser_url(Object.keys(data), get_save_opened_facts(data))
    window.location.reload()
  }
</script>

<div class="bar" class:border={opened}>
  <button onclick={toggle_open} class:hidden={opened}>Menu</button>
  <div class:hidden={!opened}>
    <button onclick={toggle_open}>Close</button>
    <br />

    <button type="button" onclick={on_file_upload_click}>Upload save file</button>
    <input
      bind:this={input}
      id="fileinput"
      type="file"
      accept=".owsave"
      class="hidden"
      onchange={handle_file_change} />
  </div>
</div>

<style lang="scss">
  $color: #3280ff;

  .bar {
    position: absolute;
    top: 0;
    left: 0;

    height: auto;
    padding: 1em 1em;

    color: white;

    // above map
    z-index: 1000;
  }
  /*.border {
    border: 1px $color solid;
  }*/
  button {
    background: #082638;
    cursor: pointer;
    margin-bottom: 5px;
  }
  .hidden {
    display: none;
  }
</style>
