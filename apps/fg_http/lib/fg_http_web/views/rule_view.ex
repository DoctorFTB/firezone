defmodule FgHttpWeb.RuleView do
  use FgHttpWeb, :view

  def protocol_options_for_select do
    RuleProtocolEnum.__enum_map__()
  end
end
