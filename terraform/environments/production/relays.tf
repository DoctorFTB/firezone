module "relays" {
  count = var.relay_token != null ? 1 : 0

  source     = "../../modules/google-cloud/apps/relay"
  project_id = module.google-cloud-project.project.project_id

  # TODO: Remember to update the following published documentation when this changes:
  #  - /website/src/app/kb/deploy/gateways/readme.mdx
  #  - /website/src/app/kb/architecture/tech-stack/readme.mdx
  instances = {
    "asia-east1" = {
      cidr_range = "10.129.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["asia-east1-c"]
    }

    "asia-south1" = {
      cidr_range = "10.130.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["asia-south1-c"]
    }

    "australia-southeast1" = {
      cidr_range = "10.131.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["australia-southeast1-c"]
    }

    "europe-west1" = {
      cidr_range = "10.132.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["europe-west1-d"]
    }

    # "me-central1" = {
    #   cidr_range = "10.133.0.0/24"
    #   type       = "e2-micro"
    #   replicas   = 2
    #   zones      ["me-central1-a"]
    # }

    "southamerica-east1" = {
      cidr_range = "10.134.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["southamerica-east1-c"]
    }

    "us-central1" = {
      cidr_range = "10.135.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["us-central1-f"]
    }

    "us-east1" = {
      cidr_range = "10.136.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["us-east1-c"]
    }

    "us-west2" = {
      cidr_range = "10.137.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["us-west2-b"]
    }

    "europe-central2" = {
      cidr_range = "10.138.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["europe-central2-c"]
    }

    "europe-north1" = {
      cidr_range = "10.139.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["europe-north1-c"]
    }

    "europe-west2" = {
      cidr_range = "10.140.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["europe-west2-c"]
    }

    "us-east4" = {
      cidr_range = "10.141.0.0/24"
      type       = "e2-micro"
      replicas   = 2
      zones      = ["us-east4-c"]
    }
  }

  container_registry = module.google-artifact-registry.url

  image_repo = module.google-artifact-registry.repo
  image      = "relay"
  image_tag  = local.relay_image_tag

  observability_log_level = "info,hyper=off,h2=warn,tower=warn"

  application_name    = "relay"
  application_version = replace(local.relay_image_tag, ".", "-")

  health_check = {
    name     = "health"
    protocol = "TCP"
    port     = 8080

    initial_delay_sec = 60

    check_interval_sec  = 15
    timeout_sec         = 10
    healthy_threshold   = 1
    unhealthy_threshold = 3

    http_health_check = {
      request_path = "/healthz"
    }
  }

  api_url = "wss://api.${local.tld}"
  token   = var.relay_token
}

# Allow SSH access using IAP for relays
resource "google_compute_firewall" "relays-ssh-ipv4" {
  count = length(module.relays) > 0 ? 1 : 0

  project = module.google-cloud-project.project.project_id

  name    = "relays-ssh-ipv4"
  network = module.relays[0].network

  allow {
    protocol = "tcp"
    ports    = [22]
  }

  allow {
    protocol = "udp"
    ports    = [22]
  }

  allow {
    protocol = "sctp"
    ports    = [22]
  }

  log_config {
    metadata = "INCLUDE_ALL_METADATA"
  }

  # Only allows connections using IAP
  source_ranges = local.iap_ipv4_ranges
  target_tags   = module.relays[0].target_tags
}

# Trigger an alert when there is at least one region without a healthy relay
resource "google_monitoring_alert_policy" "connected_relays_count" {
  project = module.google-cloud-project.project.project_id

  display_name = "Relays are down"
  combiner     = "OR"

  notification_channels = module.ops.notification_channels

  conditions {
    display_name = "Relay Instances"

    condition_threshold {
      filter     = "resource.type = \"gce_instance\" AND metric.type = \"custom.googleapis.com/elixir/domain/relays/online_relays_count/last_value\""
      comparison = "COMPARISON_LT"

      # at least one relay per region must be always online
      threshold_value = length(module.relays[0].instances)
      duration        = "0s"

      trigger {
        count = 1
      }

      aggregations {
        alignment_period     = "60s"
        cross_series_reducer = "REDUCE_MAX"
        per_series_aligner   = "ALIGN_MEAN"
      }
    }
  }

  alert_strategy {
    auto_close = "172800s"
  }
}
