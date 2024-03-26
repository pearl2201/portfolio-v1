defmodule PortfolioApiWeb.MailController do
  use PortfolioApiWeb, :controller

  def send(conn, %{"name" => name, "email" => email, "message" => message}) do
    case PortfolioApi.Mails.ContactEmail.contact(name, email, message)
         |> PortfolioApi.Mailer.deliver() do
      {:ok, _} -> render(conn, :send, data: "send success")
      {:error, err} ->
        IO.inspect(err)
        render(conn, :error, data: "send failed")
    end
  end
end
