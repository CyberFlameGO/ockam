defmodule Ockam.Hub.Kafka.TopicCleanup do
  @moduledoc """
  Helper module to cleanup idle kafka topics
  """

  def find_idle_topics(options, idle_time) do
    now = System.os_time(:millisecond)
    expired_time = now - idle_time

    Ockam.Kafka.get_topics(options)
    |> Enum.filter(fn topic ->
      idle_topic?(topic, options, expired_time)
    end)
    |> Enum.map(fn {topic_name, _} -> topic_name end)
  end

  def cleanup_topics(topics, options) do
    Ockam.Kafka.delete_topics(topics, options)
  end

  def idle_topic?({topic_name, partitions}, options, expired_time) do
    Enum.all?(:lists.seq(0, partitions - 1), fn partition ->
      idle_partition?(topic_name, partition, options, expired_time)
    end)
  end

  def idle_partition?(topic_name, partition, options, expired_time) do
    case Ockam.Kafka.resolve_offset(topic_name, partition, :latest, options) do
      {:ok, offset} when offset > 0 ->
        case Ockam.Kafka.fetch(topic_name, partition, offset - 1, options) do
          {:ok, messages} ->
            idle_messages?(messages, expired_time)

          _other ->
            false
        end

      _other ->
        false
    end
  end

  def idle_messages?(messages, expired_time) do
    Enum.all?(messages, fn message ->
      case message do
        {:kafka_message, _, _, _, _, ts, _} ->
          ts < expired_time

        _other ->
          false
      end
    end)
  end
end
