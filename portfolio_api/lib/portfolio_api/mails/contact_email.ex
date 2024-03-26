defmodule PortfolioApi.Mails.ContactEmail do
  import  Swoosh.Email

  def contact(name, email, message) do
    new()
    |> to(Application.get_env(:portfolio_api, PortfolioApi.Mailer)[:email_dest])
    |> from({"Anh Ngoc Dev", Application.get_env(:portfolio_api, PortfolioApi.Mailer)[:username]})
    |> subject("Your pearl2201.com website have a new contact")
    |> text_body("From #{name} #{email}\n #Receive message: \n#{message}")
  end
end
