defmodule PortfolioApi.ContactEmailTest do
  use ExUnit.Case

  test "send_mail" do
    IO.inspect(Application.get_env(:portfolio_api, PortfolioApi.Mailer))
    mail =  PortfolioApi.Mails.ContactEmail.contact("Anh Ngoc", "ngoc.nguyenanh@inwave.vn", "demo email")
    x = PortfolioApi.Mailer.deliver(mail)
    IO.inspect(mail)
    IO.inspect(x)
    assert  Application.get_env(:portfolio_api, PortfolioApi.Mailer)[:email_dest] == "nguyenanhngoc.ftu@gmail.com"
  end
end
