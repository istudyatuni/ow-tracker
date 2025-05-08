<script module>
  import {
    export_save_to_browser_url,
    get_save_opened_facts,
  } from "../../lib/saves";
  import { t } from "../../lib/i18n";

  // todo: more
  const SAVE_LOCATIONS = [
    [
      "Steam, Windows",
      "%USERPROFILE%/AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves",
    ],
    [
      "Steam, Linux",
      "$HOME/.local/share/Steam/steamapps/compatdata/753640/pfx/drive_c/users/steamuser/AppData/LocalLow/Mobius Digital/Outer Wilds/SteamSaves",
    ],
  ];
</script>

<script>
  let { upload = () => {} } = $props();

  let file_upload_help_opened = $state(false);
  let input;

  function on_file_upload_click() {
    input.click();
  }
  async function handle_file_change(e) {
    let file = await e.target.files[0].text();
    let data = JSON.parse(file).shipLogFactSaves;
    let opened_facts = get_save_opened_facts(data);
    export_save_to_browser_url(Object.keys(data), opened_facts);
    upload(opened_facts);
  }
</script>

<button type="button" onclick={on_file_upload_click}
  >{$t("upload-save-file-button")}</button>
<button
  type="button"
  class="question-button"
  onclick={() => (file_upload_help_opened = !file_upload_help_opened)}
  >?</button>
<input
  bind:this={input}
  id="fileinput"
  type="file"
  accept=".owsave"
  class="hidden"
  onchange={handle_file_change} />
<br />

{#if file_upload_help_opened}
  <div class="block-wrapper">
    <h3>{$t("file-upload-help-header")}</h3>
    {#each SAVE_LOCATIONS as [platform, path]}
      <p>
        <b>{platform}:</b>
        <code>{@html path.replaceAll("/", "/<wbr />")}</code>
      </p>
    {/each}
  </div>
{/if}

<style lang="scss">
  .block-wrapper {
    max-width: 25em;

    & p {
      margin-top: 10px;
      margin-bottom: 0;
    }

    & > h3 {
      margin: 0px;
    }
  }
  .question-button {
    font-weight: 700;
  }
  code {
    word-break: break-word;
  }
  button {
    margin-bottom: 5px;
  }
</style>
