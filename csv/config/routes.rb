Rails.application.routes.draw do
  root to: 'csv#index'
  get :ruby, to: 'csv#ruby'
  get :rust, to: 'csv#rust'
end
