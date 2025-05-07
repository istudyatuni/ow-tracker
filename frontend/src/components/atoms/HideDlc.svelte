<script module>
  import { SELECTED_CATEGORIES } from "../../lib/stores";
  import { t } from "../../lib/i18n";

  const CATEGORY = "stranger";
</script>

<label>
  <input
    type="checkbox"
    name="hide-dlc"
    class={CATEGORY}
    checked={!$SELECTED_CATEGORIES[CATEGORY]}
    onchange={(e) => {
      SELECTED_CATEGORIES.set(CATEGORY, !e.target.checked);
    }} />
  {$t("hide-dlc-checkbox")}
</label>
