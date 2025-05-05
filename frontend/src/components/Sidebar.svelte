<script module>
  import CloseIcon from "~icons/tabler/x";
  import MenuIcon from "~icons/tabler/menu-2";
  import LanguageIcon from "~icons/tabler/language-hiragana";

  import { LANGUAGE_NAMES, save_language } from "../lib/language";
  import {
    export_save_to_browser_url,
    get_save_opened_facts,
  } from "../lib/saves";
  import { LANGUAGE } from "../lib/stores";
  import { t } from "../lib/i18n";
</script>

<script>
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
    let data = JSON.parse(file).shipLogFactSaves;
    export_save_to_browser_url(Object.keys(data), get_save_opened_facts(data));
    window.location.reload();
  }
  function handle_select_lang(e) {
    save_language(e.target.value);
    window.location.reload();
  }
</script>

<div class="bar above-map" class:border={opened}>
  <button onclick={toggle_open} class:hidden={opened}><MenuIcon /></button>
  <div class:hidden={!opened}>
    <button onclick={toggle_open}><CloseIcon /></button>
    <br />

    <button type="button" onclick={on_file_upload_click}
      >{$t("upload-save-file-button")}</button>
    <input
      bind:this={input}
      id="fileinput"
      type="file"
      accept=".owsave"
      class="hidden"
      onchange={handle_file_change} />
    <br />

    <select onchange={handle_select_lang}>
      {#each Object.entries(LANGUAGE_NAMES) as [key, name]}
        <option value={key} selected={$LANGUAGE === key}>{name}</option>
      {/each}
    </select>
    <span class="icon">
      <LanguageIcon width="25" height="25" />
    </span>
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
  }
  .icon {
    padding-top: 5px;
    vertical-align: middle;
    margin-left: 4px;
  }
  /*.border {
    border: 1px $color solid;
  }*/
  button,
  select {
    background: #082638;
    cursor: pointer;
    margin-bottom: 5px;
  }
  .hidden {
    display: none;
  }
</style>
