defmodule PortfolioApiWeb.MailJSON do
  def send(%{data: data}) do
    %{
      success: true,
      data: data
    }
  end

  def error(%{data: data}) do
    %{
      success: false,
      data: data
    }
  end
end
