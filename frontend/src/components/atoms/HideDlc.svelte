<script module>
  import { SELECTED_CATEGORIES, SETTINGS } from "@/lib/stores";
  import { t } from "@/lib/i18n";

  const CATEGORY = "stranger";
</script>

<label>
  <input
    type="checkbox"
    class={CATEGORY}
    bind:checked={$SETTINGS.hide_dlc}
    onchange={(e) => SELECTED_CATEGORIES.set(CATEGORY, !e.target.checked)} />
  {$t("hide-dlc-checkbox")}
</label>
