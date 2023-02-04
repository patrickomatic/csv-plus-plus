# frozen_string_literal: true

require_relative '../entities'

module CSVPlusPlus
  module Language
    module Entities
      # A basic building block of the abstract syntax tree (AST)
      class Entity
        attr_reader :id, :type

        # initialize
        def initialize(type, id: nil)
          @type = type.to_sym
          @id = id.downcase.to_sym if id
        end

        # ==
        def ==(other)
          self.class == other.class && @type == other.type && @id == other.id
        end

        # Respond to predicates that correspond to types like #boolean?, #string?, etc
        def method_missing(method_name, *_arguments)
          if method_name =~ /^(\w+)\?$/
            t = ::Regexp.last_match(1)
            a_type?(t) && @type == t.to_sym
          else
            super
          end
        end

        # support predicates by type
        def respond_to_missing?(method_name, *_arguments)
          (method_name =~ /^(\w+)\?$/ && a_type?(::Regexp.last_match(1))) || super
        end

        private

        def a_type?(str)
          ::CSVPlusPlus::Language::TYPES.include?(str.to_sym)
        end
      end

      # An entity that can take arguments
      class EntityWithArguments < Entity
        attr_reader :arguments

        # initialize
        def initialize(type, id: nil, arguments: [])
          super(type, id:)
          @arguments = arguments
        end

        # ==
        def ==(other)
          super && @arguments == other.arguments
        end

        protected

        attr_writer :arguments

        def arguments_to_s
          @arguments.join(', ')
        end
      end
    end
  end
end
