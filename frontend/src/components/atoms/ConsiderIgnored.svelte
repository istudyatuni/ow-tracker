<script module>
  import { SETTINGS } from "../../lib/stores";
  import { t } from "../../lib/i18n";
</script>

<script>
  let { onchange = () => {} } = $props();
</script>

<label>
  <input
    type="checkbox"
    name="show-spoilers"
    class="other"
    {onchange}
    bind:checked={$SETTINGS.consider_ignored_facts} />
  {$t("consider-ignored-checkbox")}
</label>
