#!/usr/bin/env pwsh

#Requires -Version 7.0

[CmdletBinding()]
param(
  [Parameter(Mandatory = $true)]
  [ValidatePattern('^cargo_session=[A-Za-z0-9%]+$')]
  [string] $Cookie
)

$ErrorActionPreference = 'Stop'

# Get the user ID first.
$resp = Invoke-RestMethod 'https://crates.io/api/v1/me' -Headers @{Cookie = $Cookie}
$invitee_name = $resp.user.login
$invitee_id = $resp.user.id

$invitations = @()
Write-Verbose "Getting invites for $invitee_name ($invitee_id)"
$next_page = "?invitee_id=$invitee_id&per_page=100"
while ($next_page) {
  $resp = Invoke-RestMethod "https://crates.io/api/private/crate_owner_invitations$next_page" -Headers @{Cookie = $Cookie}
  $invitations += $resp.invitations
  $next_page = $resp.meta.next_page
}

$i = 1.0
foreach ($invitation in $invitations) {
  Write-Progress "Accepting $($invitations.count) invitations" -Status $invitation.crate_name -PercentComplete ($i / $invitations.count * 100)
  $body = @{
    crate_owner_invite = @{
      crate_id = $invitation.crate_id
      accepted = $true
    }
  } | ConvertTo-Json
  $null = Invoke-RestMethod "https://crates.io/api/v1/me/crate_owner_invitations/$($invitation.crate_id)" -Method Put -Headers @{Cookie = $Cookie} -ContentType 'application/json' -Body $body
  $i++
}
