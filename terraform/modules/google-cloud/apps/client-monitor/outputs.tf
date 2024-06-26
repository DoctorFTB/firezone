output "service_account" {
  value = google_service_account.application
}

output "target_tags" {
  value = local.application_tags
}

output "instance" {
  value = google_compute_instance.client_monitor
}

output "internal_ip" {
  value = google_compute_address.client_monitor.address
}
