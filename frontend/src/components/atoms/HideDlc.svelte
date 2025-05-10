<script module>
  import { SELECTED_CATEGORIES, SETTINGS } from "@/lib/stores";
  import { t } from "@/lib/i18n";

  const CATEGORY = "stranger";
</script>

<script>
  let { onchange = () => {} } = $props();
</script>

<label>
  <input
    type="checkbox"
    name="hide-dlc"
    class={CATEGORY}
    bind:checked={$SETTINGS.hide_dlc}
    onchange={(e) => {
      SELECTED_CATEGORIES.set(CATEGORY, !e.target.checked);
      onchange();
    }} />
  {$t("hide-dlc-checkbox")}
</label>
